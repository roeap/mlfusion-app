import React, { useEffect, useState } from "react";
import { mlfusion } from "../clients";

export const DataAssetCatalog: React.FC = () => {
  const [dataAssets, setDataAssets] = useState<mlfusion.AreaInfo[]>([]);

  useEffect(() => {
    mlfusion.list_data_assets().then((data) => setDataAssets(data));
  }, [dataAssets]);
  return <p>{JSON.stringify(dataAssets)}</p>;
};
