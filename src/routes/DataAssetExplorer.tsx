import React, { useState } from "react";
import Plot from "react-plotly.js";
import { Button, Layout, PageHeader, Input, Tooltip } from "antd";
import { EditOutlined, CheckOutlined } from "@ant-design/icons";

export const DataAssetExplorer: React.FC = () => {
  const [isEditable, setIsEditable] = useState(false);
  const [query, setQuery] = useState<string | undefined>(undefined);

  return (
    <Layout style={{ height: "100vh" }}>
      <PageHeader
        title="Dataset"
        extra={<Button type="primary">Execute</Button>}
      >
        <Input.Group compact>
          <Input
            placeholder="SQL query"
            disabled={!isEditable}
            style={{ width: "calc(100% - 31px)" }}
            value={query}
            onChange={(value) => setQuery(value.target.value)}
          />
          <Tooltip title={isEditable ? "submit query" : "edit query"}>
            <Button
              icon={isEditable ? <CheckOutlined /> : <EditOutlined />}
              onClick={() => setIsEditable(!isEditable)}
            />
          </Tooltip>
        </Input.Group>
      </PageHeader>
      <Plot
        style={{ width: "100%", height: "100%" }}
        config={{ responsive: true }}
        useResizeHandler={true}
        data={[
          {
            x: [1, 2, 3, 4],
            y: [2, 6, 3, 5],
            type: "scatter",
            mode: "lines+markers",
            marker: { color: "red" },
          },
          { type: "bar", x: [1, 2, 3], y: [2, 5, 3] },
        ]}
        layout={{ autosize: true }}
      />
    </Layout>
  );
};
