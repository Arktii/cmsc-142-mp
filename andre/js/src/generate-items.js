import * as fs from "fs";

const maxWeight = 1500;
const minWeight = 100;
const maxValue = 500;
const minValue = 100;

const randomInt = (min, max) =>
  Math.floor(Math.random() * (max - min + 1)) + min;

const items = [];

for (let i = 0; i < 100000; i++) {
  items.push({
    weight: randomInt(minWeight, maxWeight),
    value: randomInt(minValue, maxValue),
  });
}

const jsonString = JSON.stringify(items, null, 2);

fs.writeFile("./src/items.json", jsonString, (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("The file was saved!");
  }
});
