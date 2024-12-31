import React from "react";
import { Outlet } from "react-router-dom";

function HomeWindow() {
  return (
    <div className="main-display">
      <Outlet />
    </div>
  );
}

export default HomeWindow;
