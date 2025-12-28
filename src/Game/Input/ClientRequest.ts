import { invoke } from "@tauri-apps/api/core";
import { InputFrame } from "./InputFrame";

export type ClientRequest =
  | { type: "Add" }
  | { type: "Remove"; id: number } // PlayerId as number
  | { type: "Input"; entity_id: number; frame: InputFrame };

export function sendClientRequest(request: ClientRequest) {
  invoke("client_request", { request }).catch((err) => {
    console.warn("Failed to send client request:", err);
  });
}

export function sendClientRequestWithResponse<T>(
  request: ClientRequest
): Promise<T> {
  return invoke<T>("client_request", { request });
}
