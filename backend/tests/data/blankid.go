package main


func a() (int, int, int) {
	return 1, 2, 3
}

func main () {
	i, _, _ := a()
	_, j, _ := a()
	_, _, k := a()
}