const { invoke } = window.__TAURI__.core;

const startBtn = document.getElementById("start-game-btn");
const statusText = document.getElementById("status-text");

startBtn.addEventListener("click", async () => {
    statusText.textContent = "正在获取 Manifest...";
    try {
        const response = await invoke("start_game");
        statusText.textContent = response;
    } catch (error) {
        statusText.textContent = "错误: " + error;
        console.error(error);
    }
});
