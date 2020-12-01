<Query Kind="Statements" />

var items = File.ReadAllLines(@"C:\Projects\GitHub\advent-of-code-2020\src\components\day1.dat").Select(i => int.Parse(i));
var target = 2020;
items.Select(i =>  items.Select(s => new { Left = i, Right = s, Sum = i+s, Multiple = i*s })).SelectMany(i=>i).First(i=>i.Sum == target).Dump("Part 1");