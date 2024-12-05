package common

// Checks if any element of a is contained in b.
func ContainsAny(a, b []int) bool {
	bMap := make(map[int]struct{})

	for _, y := range b {
		bMap[y] = struct{}{}
	}

	for _, x := range a {
		if _, exists := bMap[x]; exists {
			return true
		}
	}

	return false
}
