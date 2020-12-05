Calculate total rebate when a rebate is added on a sliding scale.

Example.

```
From 0 to 499999, 0%
From 500000 to 999999, 8%
From 1000000 to 2499999, 9%
From 2500000 to 7499999, 11%
From 7500000 to 9999999, 12%
From 10000000 and up, 14%
```

The first element added to the array is 8%, then the delta with the second, 1%, and so on.

The loop then subtract the first volume (500000) from the total, calculate the rebate.

```
while true {
    r += (total - 500000)/total * rebate
}
```
