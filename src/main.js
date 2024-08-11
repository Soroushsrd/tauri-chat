const { invoke } = window.__TAURI__.tauri;

async function sendMessage() {
  const userInputEl = document.querySelector("#user-input");
  const chatHistoryEl = document.querySelector("#chat-history");

  const userMessage = userInputEl.value.trim();
  if (userMessage === "") return;

  // Append user message to the chat history
  appendMessage(chatHistoryEl, "You", userMessage);
  userInputEl.value = "";

  // Call the Rust function to generate the chatbot response
  const chatHistoryText = chatHistoryEl.innerText;
  const botResponse = await invoke("generate_response", { question: userMessage, chatHistory: chatHistoryText });

  // Append the chatbot's response to the chat history
  appendMessage(chatHistoryEl, "Bot", botResponse);
}

function appendMessage(chatHistoryEl, sender, message) {
  const messageEl = document.createElement("div");
  messageEl.classList.add("message");

  const senderEl = document.createElement("strong");
  senderEl.textContent = `${sender}: `;

  messageEl.appendChild(senderEl);
  messageEl.appendChild(document.createTextNode(message));
  chatHistoryEl.appendChild(messageEl);

  // Scroll to the bottom of the chat history
  chatHistoryEl.scrollTop = chatHistoryEl.scrollHeight;
}

window.addEventListener("DOMContentLoaded", () => {
  const sendButtonEl = document.querySelector("#send-button");
  sendButtonEl.addEventListener("click", sendMessage);

  const userInputEl = document.querySelector("#user-input");
  userInputEl.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      sendMessage();
    }
  });
});
