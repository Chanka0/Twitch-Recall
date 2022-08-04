# Twitch Recall
Basically a remake of [TwitchRecover](https://github.com/TwitchRecover/TwitchRecover), the popular Java lang version of this tool. The fact it used Java was something I spited, so I used the opportunity to learn Rust. A language arguably just as hard/punishing. Maybe I'm a masochist. Maybe this is a cry for help. ¯\\__(ツ)_/¯ I dunno.

## Features
- Retreives unlisted/sub-only streams within 60 days of their air date
- Multiplatform (untested, should work on Windows/MacOS/Linux)
- Easy to Use (All stream data is automatically scraped)
- Quick

## How To Use
1. Run the executable or open it in the console of your choice
2. Grab the TwitchTracker link for the stream (Streamer -> Streams -> The stream you want)
3. Copy the link into the program when prompted (should look like https://twitchtracker.com/channel/streams/stream_id)
4. Hit enter
5. Copy an available link into a network streaming application (VLC works)
6. Enjoy your content

## Planned Features
- Broader Site Support (SullyGnome, StreamCharts)
- VOD Downloading
- Clip Support
- GUI (Likely Not)

## Lil' Side Note
If a stream isn't showing up there could be a few reasons for it.
1. It has been deleted. Twitch is barely competent but it seems after months (years?) they made it so deleting VODs actually deletes them. It happens when a streamer is banned too.

2. It's >= 60 days old (deleted)
2. Outdated domains

## Credits
@daylamtayari for creating [TwitchRecover](https://github.com/TwitchRecover/TwitchRecover)

@ItIckeYd for gathering a domain name list for scraping VODS