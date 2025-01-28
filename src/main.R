data <- c(1, 2, 3, 4, 5)

print(t.test(data, alternative = "greater", mu = 0, conf.level = 0.95))
