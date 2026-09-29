import dotnet from "./assets/icons/dotnet.svg?raw";
import vite from "./assets/icons/vite.svg?raw";
import astro from "./assets/icons/astro.svg?raw";
import rust from "./assets/icons/rust.svg?raw";

export type Icon = { svg: string; color: string };
const ICONS = {
  dotnet: { svg: dotnet, color: "#512BD4" },
  vite: { svg: vite, color: "#646CFF" },
  astro: { svg: astro, color: "#BC52EE" },
  rust: { svg: rust, color: "currentColor" },
} satisfies Record<string, Icon>;

// Works out which tool a process is from its name and command.
export function detectIcon(process: string, command: string, shortCommand: string): Icon | undefined {
  const tool = shortCommand.toLowerCase();

  if (tool.startsWith("vite")) return ICONS.vite;
  if (tool.startsWith("astro")) return ICONS.astro;

  // "dotnet run" starts the app from bin\Debug\net8.0\ (or Release).
  if (/^dotnet(\.exe)?$/i.test(process) || /[\/]bin[\/](debug|release)[\/]net\d/i.test(command)) {
    return ICONS.dotnet;
  }

  // "cargo run" starts the app from target\debug\ (or release).
  if (/^cargo(\.exe)?$/i.test(process) || /[\/]target[\/](debug|release)[\/]/i.test(command)) {
    return ICONS.rust;
  }
}
