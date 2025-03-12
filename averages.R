# Calculates the mean of the answers from each party to each question

process <- function(file) {
  options(width = 10000)

  csv <- read.csv(file)
  data <- subset(csv, Party != "")
  subsets <- split(data, data$Party)
  averages <- lapply(subsets, function(subset) {
    colMeans(subset[, 8:32], na.rm = TRUE)
  })
  
  table <- do.call(rbind, averages)
  colnames(table) <- 1:25
  rownames(table) <- names(averages)

  print(table)
}

process("COUNTY_ELECTIONS_2025.csv")
process("MUNICIPAL_ELECTIONS_2025.csv")
