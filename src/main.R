library(BSDA)

sample_data <- c(1, 2, 3, 4, 5)

print(
  z.test(
    x = sample_data,
    alternative = "less",
    mu = 0,
    sigma.x = sd(sample_data)
  )
)
