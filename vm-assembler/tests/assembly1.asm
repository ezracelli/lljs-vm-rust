start:

	mov $4200, r1
	mov r1, &0060
	mov $1300, r1
	mov &0060, r2

	add  r1, r2

	hlt
