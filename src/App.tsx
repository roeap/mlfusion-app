import { Layout, Menu } from "antd";
import { Route, Link, Routes } from "react-router-dom";
import React, { useState } from "react";
import "./App.css";
import { SearchOutlined, DatabaseOutlined } from "@ant-design/icons";
import { DataAssetExplorer, DataAssetCatalog } from "./routes";

const { Content, Sider } = Layout;

const App: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);

  const onCollapse = (value: boolean) => {
    setCollapsed(value);
  };

  const leftMargin = collapsed ? 80 : 200;

  return (
    <Layout hasSider>
      <Sider
        collapsible
        collapsed={collapsed}
        onCollapse={onCollapse}
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
            <SearchOutlined />
            <span>Catalog</span>
            <Link to="/" />
          </Menu.Item>
          <Menu.Item key="2">
            <DatabaseOutlined />
            <span>Dataset</span>
            <Link to="/meseros" />
          </Menu.Item>
        </Menu>
      </Sider>
      <Layout
        className="site-layout"
        style={{ marginLeft: leftMargin, height: "100vh" }}
      >
        <Content style={{ overflow: "scroll" }}>
          <Routes>
            <Route path="/" element={<DataAssetCatalog />} />
            <Route path="meseros" element={<DataAssetExplorer />} />
          </Routes>
        </Content>
      </Layout>
    </Layout>
  );
};

export default App;
