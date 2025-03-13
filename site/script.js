function format(values) {
  const title = "The most suitable parties are listed in order from most to least suitable. A lower value represents a more suitable party.\n\n"
  const formatted = values.map(inner => inner.join(": ")).join("\n")
  return title + formatted
}

function sort(values) {
  return values.sort((a, b) => {
    if (a[1] < b[1]) return -1
    if (a[1] > b[1]) return 1
    return 0
  })
}

function calculate_mses(elections, answers) {
  const mses = []
  for (const [party, values] of Object.entries(elections)) {
    let mse = 0
    let count = 0
    for (let i = 0; i < 25; ++i) {
      if (answers[i] == null) {
        continue
      }
      mse += (values[i] - answers[i]) ** 2
      ++count
    }
    mse = (mse / count).toFixed(2)
    mses.push([party, mse])
  }
  return mses
}

function get_answers() {
  const answers = []
  for (let i = 0; i < 25; ++i) {
    const selected = document.querySelector(`input[name="q${i}"]:checked`)
    answers.push(selected ? selected.value : null)
  }
  return answers
}

function submit(elections) {
  const answers = get_answers()
  const mses = calculate_mses(elections, answers)
  const sorted = sort(mses)
  const formatted = format(sorted)
  window.alert(formatted)
}
