<Query Kind="Statements" />

var items = File.ReadAllLines(@"C:\Projects\GitHub\advent-of-code-2020\src\components\day1.dat").Select(i => int.Parse(i)).ToArray();
var target = 2020;
var bicost = items.Select(i => items.Select(s => new { Items = new[] { i, s}, Sum = i+s, Multiple = i*s })).SelectMany(i=>i);
bicost.First(i=>i.Sum == target).Dump("Part 1");

var tricost = bicost.Select(i => items.Select(s => new { Items = i.Items.Union(new[] { s}), Sum = i.Sum + s, Multiple = i.Multiple * s})).SelectMany(i => i);
tricost.First(i => i.Sum == target).Dump("Part 2");

