import fs from "fs";
import path from "path";
import process from "process";

import yargs from "yargs/yargs";
import { hideBin } from "yargs/helpers";

import Results from "./Results";

type EntryPoint = (rawInput: Buffer) => Results;

const PROJECT_ROOT = path.join(__dirname, "..");
const INPUT_ROOT = path.join(PROJECT_ROOT, "inputs");
const DAY_CODE_ROOT = path.join(PROJECT_ROOT, "out", "days");

function getInputPath(day: number): string {
  return path.join(INPUT_ROOT, `day${day}.txt`);
}

function getCodePath(day: number): string {
  return path.join(DAY_CODE_ROOT, `day${day}`, "main.js");
}

async function loadInput(day: number): Promise<Buffer> {
  const inputPath = getInputPath(day);

  if (!fs.existsSync(inputPath))
    throw new Error(`No input file found for day ${day}`);

  return fs.promises.readFile(inputPath);
}

async function loadEntryPoint(day: number): Promise<EntryPoint> {
  const codePath = getCodePath(day);

  if (!fs.existsSync(codePath))
    throw new Error(`No entry point found for day ${day}`);

  const module = await import(codePath);
  if (module.default == undefined)
    throw new Error(`Module for day ${day} does not expose an entry point`);

  return module.default;
}

function main() {
  const args = yargs(hideBin(process.argv))
    .option("day", {
      alias: "d",
      demandOption: true,
      number: true,
    })
    .parseSync();

  Promise.all([loadInput(args.day), loadEntryPoint(args.day)])
    .then(([inputData, entryPoint]) => entryPoint(inputData))
    .then((results) =>
      console.log(`Part 1: ${results.part1}\nPart 2: ${results.part2}`)
    )
    .catch((reason: Error) => {
      console.error(`Error: ${reason.message}`);
      process.exitCode = 1;
    });
}

main();
