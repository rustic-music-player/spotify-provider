use rustic::library::{Track, Album};
use rustic::provider;
use rspotify::spotify::model::track::{FullTrack, SimplifiedTrack};
use util::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyFullTrack(FullTrack);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifySimplifiedTrack(SimplifiedTrack);

impl From<SpotifyFullTrack> for Track {
    fn from(track: SpotifyFullTrack) -> Self {
        let track = track.0;
        let artist = artists_to_artist(track.artists);

        Track {
            id: None,
            title: track.name,
            artist_id: None,
            artist: artist.clone(),
            album_id: None,
            album: Some(Album {
                id: None,
                title: track.album.name,
                artist_id: None,
                artist,
                provider: provider::Provider::Spotify,
                image_url: convert_images(&track.album.images),
                uri: format!("spotify://album/{}", track.album.id)
            }),
            stream_url: String::new(),
            provider: provider::Provider::Spotify,
            image_url: convert_images(&track.album.images),
            uri: format!("spotify://track/{}", track.id),
            duration: Some(track.duration_ms as u64)
        }
    }
}

impl From<SpotifySimplifiedTrack> for Track {
    fn from(track: SpotifySimplifiedTrack) -> Self {
        let track = track.0;
        let artist = artists_to_artist(track.artists);

        Track {
            id: None,
            title: track.name,
            artist_id: None,
            artist,
            album_id: None,
            album: None,
            stream_url: String::new(),
            provider: provider::Provider::Spotify,
            image_url: None,
            uri: format!("spotify://track/{}", track.id),
            duration: Some(track.duration_ms as u64)
        }
    }
}

impl From<FullTrack> for SpotifyFullTrack {
    fn from(track: FullTrack) -> Self {
        SpotifyFullTrack(track)
    }
}

impl From<SimplifiedTrack> for SpotifySimplifiedTrack {
    fn from(track: SimplifiedTrack) -> Self {
        SpotifySimplifiedTrack(track)
    }
}