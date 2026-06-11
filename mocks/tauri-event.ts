// Mock for @tauri-apps/api/event
// Provides listen() that returns a no-op unlisten function

type UnlistenFn = () => void;
type EventCallback<T> = (event: { payload: T }) => void;

export async function listen<T>(
  event: string,
  handler: EventCallback<T>
): Promise<UnlistenFn> {
  console.debug(`[mock:event] listen("${event}") registered`);
  return () => {
    console.debug(`[mock:event] listen("${event}") unregistered`);
  };
}
