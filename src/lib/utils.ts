type DurationFormat =
  | "mm:ss.fff"
  | "mm:ss"
  | "hh:mm:ss.fff"
  | "hh:mm:ss"
  | "ss.fff";

export function formatDuration(
  totalMs: number,
  format: DurationFormat = "mm:ss.fff",
): string {
  const hasHours = format.includes("hh");
  const hasMinutes = format.includes("mm");

  const parts: Record<string, string> = {
    hh: String(Math.floor(totalMs / 3_600_000)).padStart(2, "0"),
    mm: String(
      Math.floor(totalMs / 60_000) % (hasHours ? 60 : Infinity),
    ).padStart(2, "0"),
    ss: String(
      Math.floor(totalMs / 1000) % (hasMinutes ? 60 : Infinity),
    ).padStart(2, "0"),
    fff: String(Math.floor(totalMs % 1000)).padStart(3, "0"),
  };

  return format.replace(/hh|mm|ss|fff/g, (token) => parts[token]);
}
