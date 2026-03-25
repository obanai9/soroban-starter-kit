import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ConnectivityProvider } from "./context/ConnectivityContext";
import { StorageProvider } from "./context/StorageContext";
import { TransactionQueueProvider } from "./context/TransactionQueueContext";
import { ThemeProvider } from "./context/ThemeContext";
import { TutorialProvider } from "./context/TutorialContext";
import { PWAProvider } from "./context/PWAContext";
import { LocalizationProvider } from "./context/LocalizationContext";
import "./styles/index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <LocalizationProvider>
      <ThemeProvider>
        <TutorialProvider>
          <PWAProvider>
            <ConnectivityProvider>
              <StorageProvider>
                <TransactionQueueProvider>
                  <App />
                </TransactionQueueProvider>
              </StorageProvider>
            </ConnectivityProvider>
          </PWAProvider>
        </TutorialProvider>
      </ThemeProvider>
    </LocalizationProvider>
  </React.StrictMode>,
);
