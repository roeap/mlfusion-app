import React, { useEffect, useState } from "react";
import { mlfusion } from "../clients";
import { Card, List } from "antd";

export const DataAssetCatalog: React.FC = () => {
  const [dataAssets, setDataAssets] = useState<mlfusion.AreaInfo[]>([]);

  useEffect(() => {
    mlfusion.list_data_assets().then((data) => setDataAssets(data));
  }, [dataAssets]);
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
          <Card title={item.source.location?.name}>Card content</Card>
        </List.Item>
      )}
    />
  );
};
