// Mock for @tauri-apps/api/dpi
// Used by BubbleView.tsx via dynamic import()

export class LogicalSize {
  constructor(public width: number, public height: number) {}
}
export class LogicalPosition {
  constructor(public x: number, public y: number) {}
}
export class PhysicalSize {
  constructor(public width: number, public height: number) {}
}
export class PhysicalPosition {
  constructor(public x: number, public y: number) {}
}
