use crate::{errors::MlFusionError, models};
use arrow_flight::{
    flight_descriptor::DescriptorType, flight_service_client::FlightServiceClient, Criteria,
    FlightDescriptor, FlightInfo,
};
use futures::{stream::Stream, StreamExt, TryStreamExt};
use log::debug;
use prost::Message;

#[derive(Clone)]
pub struct MLFusionClient {
    flight_client: FlightServiceClient<tonic::transport::channel::Channel>,
}

impl MLFusionClient {
    /// Create a new MLFusionClient to connect to mlfusion server listening
    /// on the specified host and port
    pub async fn try_new(host: &str, port: u16) -> Result<Self, MlFusionError> {
        let addr = format!("http://{}:{}", host, port);
        debug!("MLFusionClient connecting to {}", addr);
        let flight_client = FlightServiceClient::connect(addr.clone())
            .await
            .map_err(|e| MlFusionError::Generic {
                source: Box::new(e),
            })?;
        debug!("MLFusionClient connected OK");

        Ok(Self { flight_client })
    }

    pub async fn list_data_assets(
        &mut self,
    ) -> Result<Vec<models::AreaSourceReference>, MlFusionError> {
        let criterial = Criteria::default();
        let flight_stream: FlightInfoStream = self
            .flight_client
            .list_flights(criterial)
            .await
            .map_err(|e| MlFusionError::Generic {
                source: Box::new(e),
            })?
            .into_inner()
            .into();

        let area_stream = flight_stream.map_ok(|info| {
            if let Some(descriptor) = info.flight_descriptor {
                // TODO remove panic
                models::AreaSourceReference::decode(descriptor.cmd.as_ref()).unwrap()
            } else {
                models::AreaSourceReference {
                    ..Default::default()
                }
            }
        });
        Ok(area_stream.try_collect::<Vec<_>>().await?)
    }

    pub async fn get_data_asset_info(
        &mut self,
        source: models::AreaSourceReference,
    ) -> Result<models::AreaSourceDetails, MlFusionError> {
        let descriptor = FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: source.encode_to_vec(),
            ..FlightDescriptor::default()
        };
        let flight_info = self
            .flight_client
            .get_flight_info(descriptor)
            .await
            .map_err(|e| MlFusionError::Generic {
                source: Box::new(e),
            })?
            .into_inner();

        match flight_info.flight_descriptor {
            Some(descriptor) => Ok(models::AreaSourceDetails::decode(descriptor.cmd.as_ref())
                .map_err(|err| MlFusionError::Decode { source: err })?),
            _ => Err(MlFusionError::MissingData),
        }
    }
}

pub struct FlightInfoStream {
    stream: tonic::Streaming<FlightInfo>,
}

impl FlightInfoStream {
    pub fn new(stream: tonic::Streaming<FlightInfo>) -> Self {
        Self { stream }
    }
}

impl Stream for FlightInfoStream {
    type Item = Result<FlightInfo, MlFusionError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx).map(|x| match x {
            Some(flight_data_chunk_result) => {
                let converted_chunk = flight_data_chunk_result
                    .map_err(|e| MlFusionError::from_external_error(Box::new(e)));
                // .and_then(|flight_data_chunk| {
                //     flight_data_to_arrow_batch(
                //         &flight_data_chunk,
                //         self.schema.clone(),
                //         &self.dictionaries_by_id,
                //     )
                // });
                Some(converted_chunk)
            }
            None => None,
        })
    }
}

impl From<tonic::Streaming<FlightInfo>> for FlightInfoStream {
    fn from(stream: tonic::Streaming<FlightInfo>) -> Self {
        Self::new(stream)
    }
}
