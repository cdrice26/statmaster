# Chi-Square Tests Example

# Goodness of Fit Test
# Example: Testing if a die is fair
gof_observed <- c(30, 25, 20, 15, 25, 35) # Observed frequencies
gof_expected <- c(25, 25, 25, 25, 25, 25)

# Print raw data for Goodness of Fit Test
cat("Goodness of Fit Test - Raw Data:\n")
cat("Observed Frequencies:", gof_observed, "\n")
cat("Expected Frequencies:", gof_expected, "\n\n")

# Perform Chi-Square Goodness of Fit Test
gof_test <- chisq.test(x = gof_observed, p = gof_expected / sum(gof_expected))
print("Chi-Square Goodness of Fit Test Results:")
print(gof_test)

# Print expected counts for Goodness of Fit Test
cat("\nGoodness of Fit Test - Expected Counts:\n")
print(gof_test$expected)

# Independence Test
# Example: Testing relationship between education level and income category
independence_table <- matrix(c(
  10, 20, 30, # Low income
  15, 25, 20, # Medium income
  5, 10, 15 # High income
), nrow = 3, byrow = TRUE)
rownames(independence_table) <- c("High School", "Bachelors", "Graduate")
colnames(independence_table) <- c("Low", "Medium", "High")

# Print raw data for Independence Test
cat("\nIndependence Test - Raw Data:\n")
print(independence_table)

# Perform Chi-Square Independence Test
independence_test <- chisq.test(independence_table)
print("Chi-Square Independence Test Results:")
print(independence_test)

# Print expected counts for Independence Test
cat("\nIndependence Test - Expected Counts:\n")
print(independence_test$expected)
