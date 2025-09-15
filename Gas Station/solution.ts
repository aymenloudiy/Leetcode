function canCompleteCircuit(gas: number[], cost: number[]): number {
  let gasTank: number = 0;
  let result: number = 0;
  let total: number = 0;
  for (let i: number = 0; i < gas.length; i++) {
    gasTank += gas[i] - cost[i];
    total += gas[i] - cost[i];
    if (gasTank < 0) {
      result = i + 1;
      gasTank = 0;
    }
  }
  return total < 0 ? -1 : result;
}
