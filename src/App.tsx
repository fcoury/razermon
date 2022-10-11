import { Box } from "@chakra-ui/react";
import { Outlet } from "react-router-dom";
import "./App.css";

function App() {
  return (
    <div className="App">
      <Box h="calc(100vh)">
        <Outlet />
      </Box>
    </div>
  );
}

export default App;
