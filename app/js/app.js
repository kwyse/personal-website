var dates = document.getElementsByClassName("date");
for (var i = 0; i < dates.length; i++) {
  var date = new Date(dates[i].innerText);
  var dateOptions = { weekday: "long", day: "numeric", month: "long", year: "numeric" };
  dates[i].innerText = date.toLocaleDateString("en-gb", dateOptions);
}
