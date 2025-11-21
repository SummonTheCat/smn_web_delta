use s3::{Bucket, Region, creds::Credentials};

pub fn generate_bucket() -> Bucket {
    // Load DO Spaces credentials
    let access_key =
        std::env::var("DO_SPACES_KEY").expect("Environment variable DO_SPACES_KEY is missing");
    let secret_key = std::env::var("DO_SPACES_SECRET")
        .expect("Environment variable DO_SPACES_SECRET is missing");

    // DO Spaces config
    let bucket_name = "smn-site-storage";
    let region_name = "sfo3";
    let endpoint = "https://sfo3.digitaloceanspaces.com";

    // Credentials
    let creds = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
        .expect("Failed to create DO Spaces credentials");

    // Construct Region
    let region = Region::Custom {
        region: region_name.to_string(),
        endpoint: endpoint.to_string(),
    };

    // Initialize bucket client
    let bucket = Bucket::new(bucket_name, region, creds)
        .expect("Failed to initialize bucket")
        .with_path_style();

    *bucket
}