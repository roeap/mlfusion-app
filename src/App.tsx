import type { MenuProps } from "antd";
import { Layout, Menu, Button, Descriptions, PageHeader } from "antd";
import { Route, Link, Routes } from "react-router-dom";
import React from "react";
import Plot from "react-plotly.js";
import "./App.css";
import { PieChartTwoTone, DesktopOutlined } from "@ant-design/icons";

const { Content, Sider } = Layout;

const App: React.FC = () => (
  <Layout hasSider>
    <Sider
      style={{
        overflow: "auto",
        height: "100vh",
        position: "fixed",
        left: 0,
        top: 0,
        bottom: 0,
      }}
    >
      <div className="logo" />
      <Menu theme="dark" mode="inline" defaultSelectedKeys={["1"]}>
        <Menu.Item key="1">
          <PieChartTwoTone />
          <span>Deshboard</span>
          <Link to="/" />
        </Menu.Item>
        <Menu.Item key="2">
          <DesktopOutlined />
          <span>Meseros</span>
          <Link to="/meseros" />
        </Menu.Item>
      </Menu>
    </Sider>
    <Layout
      className="site-layout"
      style={{ marginLeft: 200, height: "100vh" }}
    >
      <PageHeader
        ghost={false}
        onBack={() => window.history.back()}
        title="Title"
        subTitle="This is a subtitle"
        extra={[
          <Button key="1" type="primary">
            Primary
          </Button>,
        ]}
      >
        <Descriptions size="small" column={3}>
          <Descriptions.Item label="Created">Lili Qu</Descriptions.Item>
          <Descriptions.Item label="Association">
            <a>421421</a>
          </Descriptions.Item>
          <Descriptions.Item label="Creation Time">
            2017-01-10
          </Descriptions.Item>
          <Descriptions.Item label="Effective Time">
            2017-10-10
          </Descriptions.Item>
          <Descriptions.Item label="Remarks">
            Gonghu Road, Xihu District, Hangzhou, Zhejiang, China
          </Descriptions.Item>
        </Descriptions>
      </PageHeader>
      <Content>
        <Routes>
          <Route path="/" element={<PlotElem />} />
          <Route path="meseros" element={<PlotElem2 />} />
        </Routes>
      </Content>
    </Layout>
  </Layout>
);

const PlotElem: React.FC = () => (
  <Plot
    style={{ width: "100%", height: "100%" }}
    config={{ responsive: true }}
    useResizeHandler={true}
    data={[
      {
        x: [1, 2, 3],
        y: [2, 6, 3],
        type: "scatter",
        mode: "lines+markers",
        marker: { color: "red" },
      },
      { type: "bar", x: [1, 2, 3], y: [2, 5, 3] },
    ]}
    layout={{ autosize: true }}
  />
);

const PlotElem2: React.FC = () => (
  <Plot
    style={{ width: "100%", height: "100%" }}
    config={{ responsive: true }}
    useResizeHandler={true}
    data={[
      {
        x: [1, 2, 3, 4],
        y: [2, 6, 3, 4],
        type: "scatter",
        mode: "lines+markers",
        marker: { color: "red" },
      },
      { type: "bar", x: [1, 2, 3], y: [2, 5, 3] },
    ]}
    layout={{ autosize: true }}
  />
);

export default App;
