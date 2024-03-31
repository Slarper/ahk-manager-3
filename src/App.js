import React, { useEffect, useState } from "react";
import './App.css';

import { invoke } from '@tauri-apps/api'

function App() {
  const [folderPath, setFolderPath] = useState("C:\\Users\\naive\\Desktop\\");
  // structure:
  // [{script:string, isRunning:bool},...]
  const [scriptList, setScriptList] = useState([]);


  useEffect(() => {
    async function scanFolder() {
      try {
        const response = await invoke("scan_folder", {
          folderPath: folderPath
        });

        const updatedScriptList = response.map((script2) => ({
          script: script2,
          isRunning: false
        }));
        setScriptList(updatedScriptList);
      } catch (error) {
        console.error(error);
      }
    }

    scanFolder();
  }, [folderPath]);

  async function toggleScript(filePath, isRunning, index) {
    try {
      if (!isRunning) {
        await invoke("run_script", { filePath });
        const updatedScriptList = [...scriptList];
        updatedScriptList[index] = { ...updatedScriptList[index], isRunning: true };
        setScriptList(updatedScriptList);
      } else {
        await invoke("shutdowm_script", { filePath });
        const updatedScriptList = [...scriptList];
        updatedScriptList[index] = { ...updatedScriptList[index], isRunning: false };
        setScriptList(updatedScriptList);
      }
    } catch (error) {
      console.error(error);
    }
  }

  return (
    <>
      <h1>AutoHotKey Manager v3.x</h1>
      {/* <p>This is some content below the input field.</p> */}

      <div id="folder-path-input">
        <input
          type="text"
          id="top-input"
          placeholder="Type something..."
          value={folderPath}
          onChange={(e) => setFolderPath(e.target.value)}
        />
      </div>

      <div id="script-zone">
        {scriptList.map((script, index) => (
          <div
            key={index}
            data-is-running={script.isRunning}
          // onClick={() => toggleScript(script.script, script.isRunning, index)}
          >
            <p
              className={script.isRunning ? "script-instance-running" : "script-instance"}
              // align-subnode left and right
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center"
              }}
            >
              <div>
                {script.script}
              </div>
              <div
                // with some gap between the two buttons
                // style={{
                //   display: "flex",
                //   justifyContent: "space-between",
                //   alignItems: "center"
                // }}
                style={{
                  display: "flex",
                  gap: "10px"

                }}
              >
                {/* Add a clickable button, positioned at the rightmost position */}

                <div className="script-instance-container">
                  <button
                    className="rightmost-button"
                    onClick={() => toggleScript(script.script, script.isRunning, index)}
                  >
                    {/* {"Run"} */}
                    {/* run or stop depend on script.isrunnning */}
                    {script.isRunning ? "Stop" : "Run"}
                  </button>
                </div>
                <div className="script-instance-container">
                  <button
                    className="rightmost-button"
                    onClick={() => invoke("vscode_script", { filePath: script.script })}
                  >
                    {"Code"}
                  </button>
                </div>
              </div>

            </p>

          </div>
        ))}
      </div>
    </>
  );
}

export default App;
