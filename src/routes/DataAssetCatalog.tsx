import React, { useEffect, useState } from "react";
import { mlfusion } from "../clients";
import { Card, List, Descriptions, Button } from "antd";

export const DataAssetCatalog: React.FC = () => {
  const [dataAssets, setDataAssets] = useState<mlfusion.AreaInfo[]>([]);

  useEffect(() => {
    mlfusion.list_data_assets().then((data) => setDataAssets(data));
  }, []);
  return <DataAssetList assets={dataAssets} />;
};

interface DataAssetListProps {
  assets: mlfusion.AreaInfo[];
}

const DataAssetList: React.FC<DataAssetListProps> = (props) => {
  return (
    <List
      style={{ margin: 16 }}
      grid={{ gutter: 16, column: 1 }}
      dataSource={props.assets}
      renderItem={(item) => (
        <List.Item>
          <DataAssetItem item={item} />
        </List.Item>
      )}
    />
  );
};

interface DataAssetItemProps {
  item: mlfusion.AreaInfo;
}

const DataAssetItem: React.FC<DataAssetItemProps> = (props) => {
  return (
    <Card>
      <Descriptions
        title={props.item.source.location?.name}
        bordered
        size="small"
        extra={<Button type="primary">Explore</Button>}
      >
        <Descriptions.Item label="Total records">100</Descriptions.Item>
        <Descriptions.Item label="Description">empty</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
