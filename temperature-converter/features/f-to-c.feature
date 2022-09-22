Feature: Convert fahrenheit to celsius

Scenario: 50F to Celsius
Given we want to know what 50.0F is in celsius
When we input it in the f-converter
Then we get 10.0C