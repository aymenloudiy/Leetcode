function hIndex(citations: number[]): number {
  let hIndex: number = 0;
  citations.sort((a, b) => b - a);
  for (let i: number = 0; i < citations.length; i++) {
    if (citations[i] < i + 1) {
      return hIndex;
    }
    hIndex += 1;
  }
  return hIndex;
}
