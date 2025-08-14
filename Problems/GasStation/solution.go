package gasstation
func _(gas []int, cost []int) int {
    totalGas := 0;
    totalCost := 0;
    currentTank :=0;
    maxIndex := 0;
    for i:= 0; i < len(gas);i++ {
        totalGas += gas[i];
        totalCost += cost[i];
        currentTank += gas[i]-cost[i];
        if currentTank < 0 {
            maxIndex = i + 1
            currentTank = 0
        }
    }
    if totalCost > totalGas {
        return -1
    }
    return maxIndex

}