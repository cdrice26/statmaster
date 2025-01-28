x <- c(1, 2, 3, 4, 5)
y <- c(2, 5, 4, 7, 9)

print(
  t.test(
    x = x, y = y, alternative = "two.sided", conf.level = 0.95,
    paired = TRUE
  )
)
