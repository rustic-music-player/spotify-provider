use rspotify::spotify::model::image::Image;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rustic::library::Artist;

pub fn convert_images(images: &Vec<Image>) -> Option<String> {
    images.first().map(|image| image.url.clone())
}

pub fn artists_to_artist(artists: Vec<SimplifiedArtist>) -> Option<Artist> {
    if artists.len() == 0 {
        return None;
    }
    let name = artists.into_iter()
        .map(|artist| artist.name)
        .collect::<Vec<String>>()
        .join(", ");
    Some(Artist {
        id: None,
        name,
        uri: String::new(),
        image_url: None
    })
}