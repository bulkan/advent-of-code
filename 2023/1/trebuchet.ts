export const trebuchet = (s: string): number => {
  const lines = s.split("\n");

  const sum = lines.reduce((acc, line) => {
    // this can be a for loop 🤷‍♂️
    const matches = line.match(/\d/g);
    // as we need first digit and last digit we probable could
    // use two points one from the start and one from the end to find them

    if (matches) {
      const [a, b] = [matches[0], matches[matches.length - 1]];

      return acc + parseInt(a + b);
    }

    return acc;
  }, 0);

  return sum;
};

if (import.meta.main) {
  const path = `${import.meta.dir}/calibrationValues.txt`;
  const file = Bun.file(path);
  const text = await file.text();

  const sum = trebuchet(text);
  console.log("done", sum);
}
