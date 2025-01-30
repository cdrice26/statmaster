library(BSDA)

sample_data <- c(1, 2, 3, 4, 5)
y <- c(2, 3, 4, 5, 6)

print(
  z.test(
    x = sample_data,
    y = y,
    alternative = "less",
    mu = 0,
    sigma.x = sd(sample_data),
    sigma.y = sd(y)
  )
)
