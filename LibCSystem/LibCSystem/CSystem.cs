using System.Text;

using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
namespace LibCSystem;

public class CSystem
{
    private static string SharedMemoryPath
    = "/dev/c/lib/csystem/";

    private static async Task Request(
        JObject JObject)
    {
        while (true)
        {
            Random Random
            = new Random();

            int Index
            = Random
            .Next(0, 9999);

            StringBuilder StringBuilder
            = new StringBuilder();

            StringBuilder
            .Append(SharedMemoryPath);
            StringBuilder
            .Append("r");
            StringBuilder
            .Append(Index);

            string Path
            = StringBuilder
            .ToString();

            bool Boolean
            = File.Exists(Path)
            == false;

            if (Boolean)
            {
                {
                    string String
                    = JObject
                    .ToString(
                    Formatting.None);

                    await File
                    .WriteAllTextAsync(
                    Path,
                    String);
                }
                break;
            }
        }
    }

    private static async Task<string> RequestAsync(
        JToken JToken)
    {
        while (true)
        {
            Random Random
            = new Random();

            int Index
            = Random
            .Next(0, 9999);

            StringBuilder StringBuilder
            = new StringBuilder();

            StringBuilder
            .Append(SharedMemoryPath);
            StringBuilder
            .Append("r");
            StringBuilder
            .Append(Index);

            string Path
            = StringBuilder
            .ToString();

            bool Boolean
            = File.Exists(Path)
            == false;

            switch (Boolean)
            {
                case true:
                    {
                        {
                            {
                                string String
                                = JToken
                                .ToString(
                                Formatting.None);

                                await File
                                .WriteAllTextAsync(
                                Path,
                                String);
                            }

                            {
                                StringBuilder
                                .Append(SharedMemoryPath);
                                StringBuilder
                                .Append(Index);

                                Path
                                = StringBuilder
                                .ToString();
                            }

                            while (true)
                            {
                                Boolean
                                = File.Exists(Path);

                                switch (Boolean)
                                {
                                    case true:
                                        {
                                            string String
                                            = await File
                                            .ReadAllTextAsync(Path);
                                            return String;
                                        }
                                }

                                await Task
                                .Yield();
                            }
                        }
                    }
            }

            await Task
            .Yield();
        }
    }

    public class Timezone
    {
        public class Zone
        {
            public string ID;
            
            internal Zone(string Zone)
            {
                ID
                = Zone;
            }
        }
        
        public static async Task<Zone[]> GetZones()
        {
            List<Zone> List
            = new List<Zone>();
            
            string String
            = await RequestAsync(
            new JObject
            {
                ["Module"]
                = "Timezone",
                
                ["Task"]
                = "List"
            });
            
            JArray JArray
            = JArray.Parse(String);

            foreach (JToken JToken
            in JArray)
            {
                String
                = (string)JToken;

                Zone Zone
                = new Zone(String);
                
                List
                .Add(Zone);
            }
            
            Array Array
            = List.ToArray();

            return (Zone[])Array;
        }

        public static async Task Set(
            Zone Zone)
        {
            await Request(
            new JObject
            {
                ["Module"]
                = "Timezone",
                
                ["Task"]
                = "Set",
                
                ["Zone"]
                = Zone.ID
            });
        }
    }
}