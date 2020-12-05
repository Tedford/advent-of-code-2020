<Query Kind="Statements" />

var credentials = new List<Dictionary<string, string>>
{
new Dictionary<string,string>()
};

File.ReadAllLines(@"C:\\Projects\\GitHub\\advent-of-code-2020\\Data\\day4.txt")
.ToList()
.ForEach(i =>
{
if (string.IsNullOrWhiteSpace(i))
{
  credentials.Add(new Dictionary<string, string>());
}
else
{
  var current = credentials.Last();
  foreach (var value in i.Split(' ').Select(t => t.Split(':')))
  {
	  current[value[0]] = value[1];
  }
}
}
);

credentials.Count().Dump("papers count");

var haircolor = new Regex(@"#[0-9a-f]{6}", RegexOptions.Compiled);
var eyecolor = new Regex("amb|blu|brn|gry|grn|hzl|oth", RegexOptions.Compiled);
var passport = new Regex("[0-9]{9}", RegexOptions.Compiled);
var height = new Regex("(?<size>[0-9]+)(?<unit>in|cm)", RegexOptions.Compiled);

bool InRange(Dictionary<string, string> credential, string key, int min, int max)
{
	var valid = false;
	if (credential.TryGetValue(key, out string s))
	{
		var value = int.Parse(s);
		valid = value > min && value < max;
	}
	return valid;
}

bool ValidHeight(Dictionary<string, string> credential)
{
	var valid = false;
	if (credential.TryGetValue("hgt", out string s))
	{
		var match = height.Match(s);

		if (match.Success)
		{
			var size = int.Parse(match.Groups["size"].Value);
			valid = match.Groups["unit"].Value switch
			{
				"cm" => size > 149 && size < 194,
				"in" => size > 58 && size < 77,
				_ => false
			};
		}
	}
	return valid;
}

bool Matches(Dictionary<string, string> credential, string key, Regex regex) => credential.TryGetValue(key, out string s) && regex.IsMatch(s);

credentials.Where(c =>
	InRange(c, "byr", 1919, 2003) &&
	InRange(c, "iyr", 2009, 2021) &&
	InRange(c, "eyr", 2019, 2031) &&
	Matches(c, "hcl", haircolor) &&
	Matches(c, "ecl", eyecolor) &&
	Matches(c, "pid", passport) &&
	ValidHeight(c)
).Count().Dump("Valid");