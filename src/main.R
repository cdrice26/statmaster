x <- c(1, 2, 3, 4, 5)
y <- c(2, 3, 4, 5, 6)

print(
  t.test(x = x, y = y, alternative = "greater", mu = 0, conf.level = 0.95)
)
