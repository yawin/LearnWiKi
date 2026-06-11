// =====================================================================
// Mock for @tauri-apps/api/core
// Routes invoke() calls to cmd-router, provides convertFileSrc
// =====================================================================

import { routeCommand } from './cmd-router';

// ===================================================================
// 主 invoke 函数 — 路由到 cmd-router
// ===================================================================
export async function invoke<T>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  console.debug(`[mock:tauri-core] invoke("${cmd}")`, args ?? '');

  try {
    const result = routeCommand<T>(cmd, args);
    // 如果是 Promise，返回它（确保 async 兼容）
    if (result instanceof Promise) {
      return result;
    }
    return result;
  } catch (err) {
    console.error(`[mock:tauri-core] Error routing "${cmd}":`, err);
    throw err;
  }
}

// ===================================================================
// convertFileSrc — 返回占位 SVG data URI
// ===================================================================
export function convertFileSrc(filePath: string): string {
  const fileName = filePath.split('/').pop() ?? 'file';
  // 生成一个带文件名的占位 SVG
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
    <rect width="200" height="200" fill="#f5f5f4"/>
    <text
      x="100" y="100"
      text-anchor="middle" dominant-baseline="central"
      font-family="system-ui, sans-serif" font-size="12" fill="#a8a29e"
    >
      ${encodeURIComponent(fileName)}
    </text>
    <rect x="20" y="140" width="160" height="1" fill="#e7e5e4"/>
    <text
      x="100" y="165"
      text-anchor="middle" dominant-baseline="central"
      font-family="system-ui, sans-serif" font-size="10" fill="#d6d3d1"
    >
      Mock File
    </text>
  </svg>`;

  // 返回 base64 编码的 data URI（SVG 的 browser-safe 编码）
  return `data:image/svg+xml;base64,${btoa(
    unescape(encodeURIComponent(svg))
  )}`;
}

// ===================================================================
// 额外导出 Tauri core API 中可能用到的工具函数
// ===================================================================

/** 转换路径为 Tauri 兼容格式（mock 中直接返回原路径） */
export function convertFileSrcForTauri(filePath: string): string {
  return convertFileSrc(filePath);
}

/** 获取 Tauri 应用信息（mock 返回占位数据） */
export async function getAppInfo(): Promise<{
  name: string;
  version: string;
  tauriVersion: string;
}> {
  return {
    name: 'OpenWiki',
    version: '0.1.0',
    tauriVersion: '2.0.0',
  };
}

/** 获取平台信息 */
export async function getPlatform(): Promise<string> {
  return navigator.platform || 'darwin';
}

/** 获取应用数据目录路径 */
export async function getAppDataDir(): Promise<string> {
  return '/mock/app-data/OpenWiki';
}

/** 获取应用配置目录路径 */
export async function getAppConfigDir(): Promise<string> {
  return '/mock/app-config/OpenWiki';
}

/** 获取用户数据目录路径 */
export async function getUserDataDir(): Promise<string> {
  return '/mock/user-data';
}
