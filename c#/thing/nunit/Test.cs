using NUnit.Framework;
using System;
using System.Text;

namespace nunit
{
    [TestFixture()]
    public class Test
    {
        [Test()]
        public void TestDeJsonDeserialize()
        {
            var expected = new Game {
                id = "ifj438ut984jf9j3f98f43",
                name = "Death Stranding",
                cost = 20,
                isAvailable = true,
                under_name = "823498fj3498j4"
            };

            // Create a string json object.
            // isAvailable has been filled in with expected value
            var gameTemplate = "{{\"id\": \"{0}\", \"name\": \"{1}\", \"cost\": {2}, \"isAvailable\": true, \"under_name\": \"{3}\"}}";
            var gameStr = String.Format(gameTemplate, expected.id, expected.name, expected.cost, expected.under_name);

            Console.WriteLine(gameStr);
            // Deserialize into that class
            var actual = Deserialize.To<Game>(gameStr);

            // Assert that certain fields are set in the class
            Assert.AreEqual(expected.id, actual.id);
            Assert.AreEqual(expected.name, actual.name);
            Assert.AreEqual(expected.cost, actual.cost);
            Assert.AreEqual(expected.isAvailable, actual.isAvailable);
            Assert.AreEqual(expected.under_name, actual.under_name);
        }

        [Test()]
        public void TestDeJsonDeserializeIncorrectFieldName()
        {
            var expected = new Game {
                id = "ifj438ut984jf9j3f98f43",
                name = "Death Stranding",
                cost = 20,
                isAvailable = true
            };

            // Create a string json object
            // The field `Name` does not exist as a field in the Game class
            var gameTemplate = "{{\"id\":\"{0}\",\"Name\":\"{1}\",\"cost\":{2},\"isAvailable\":{3}}}";
            var gameStr = String.Format(gameTemplate, expected.id, expected.name, expected.cost, expected.isAvailable);

            Console.WriteLine(gameStr);

            // Deserialize into that class
            var actual = Deserialize.To<Game>(gameStr);

            // Assert that certain fields are set in the class
            Assert.AreEqual(expected.id, actual.id);
            Assert.AreNotEqual(expected.name, actual.name);
            Assert.AreEqual(null, actual.name);
            Assert.AreEqual(expected.cost, actual.cost);
            // Assert.AreEqual(expected.isAvailable, actual.isAvailable);
        }


        [Test()]
        public void TestDeJsonSerialize()
        {
            // Create a variable of a class
            var values = new Game {
                id = "ifj438ut984jf9j3f98f43",
                name = "Death Stranding",
                cost = 20,
                isAvailable = true,
                under_name = "9238r9f8h34f"
            };

            // Set the expected output
            var expectedTemplate = "{{\"id\":\"{0}\",\"name\":\"{1}\",\"cost\":{2},\"isAvailable\":true,\"under_name\":\"{3}\"}}";
            var expected = String.Format(expectedTemplate, values.id, values.name, values.cost, values.under_name);

            // Serialize the class into a string
            var actual = Serialize.From(values);

            // Assert that the actual is equal to the expected
            Assert.AreEqual(expected, actual);
        }

        [Test()]
        public void TestDeJsonSerializeUnsetField()
        {
            // Create a variable of a class
            var values = new Game {
                id = "ifj438ut984jf9j3f98f43",
                cost = 20,
                isAvailable = true
            };

            // Set the expected output
            var expectedTemplate = "{{\"id\":\"{0}\",\"cost\":{1},\"isAvailable\":true}}";
            var expected = String.Format(expectedTemplate, values.id, values.cost);

            // Serialize the class into a string
            var actual = Serialize.From(values);

            // Assert that the actual is equal to the expected
            Assert.AreEqual(expected, actual);
        }
    }

    public class Game
    {
        public string id;
        public string name;
        public int cost;
        public bool isAvailable;
        public string under_name;
    }
}
