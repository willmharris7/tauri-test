export async function isScreenpipeRunning(): Promise<boolean> {
  try {
    const res = await fetch("http://localhost:3030/health", { signal: AbortSignal.timeout(2000) });
    return res.ok;
  } catch {
    return false;
  }
}
