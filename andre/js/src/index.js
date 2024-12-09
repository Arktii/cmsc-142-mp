import items from "./items.json" assert { type: "json" };
import { bottomUp } from "./dynamicProgramming.js";
import { createObjectCsvWriter } from "csv-writer";

const maxCapacity = 1000;
const nStart = 100;
const nEnd = 5000;
// for csv
const csvWriter = createObjectCsvWriter({
  path: "results/results.csv",
  header: [
    { id: "n", title: "n" },
    { id: "value1", title: "value1" },
    { id: "time1", title: "time1" },
    { id: "value2", title: "value2" },
    { id: "time2", title: "time2" },
    { id: "value3", title: "value3" },
    { id: "time3", title: "time3" },
    { id: "averageTime", title: "averageTime" },
  ],
});

console.log("Started: ", new Date());
for (let n = nStart; n < nEnd; n++) {
  const iterationData = [];
  let iterationTotalTime = 0;

  for (let i = 1; i <= 3; i++) {
    const startTime = Date.now();
    const { value } = bottomUp(maxCapacity, items.slice(0, n));
    const endTime = Date.now();
    const executionTime = endTime - startTime;
    iterationData.push({ [`value${i}`]: value, [`time${i}`]: executionTime });
    iterationTotalTime += executionTime;
  }

  const record = {
    n,
    ...iterationData[0],
    ...iterationData[1],
    ...iterationData[2],
    averageTime: (iterationTotalTime / 3).toFixed(6),
  };

  await csvWriter.writeRecords([record]);

  if (n % 1000 === 0) {
    console.log(`Iteration: ${n}`);
  }
}
