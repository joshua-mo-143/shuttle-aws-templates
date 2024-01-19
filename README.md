### Shuttle AWS Templates
This is a repository for storing Shuttle AWS boilerplate templates that you can use and deploy to Shuttle without any code changes. Just plug in your AWS credentials and you're good to go.

You can currently use the following:
| Template name       | Description                                                                                         | Command to init                                                              |
|---------------------|-----------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------|
| AWS S3 Microservice | A Shuttle service that initialises an S3 microservice. Can also be used with S3-compatible storage. | cargo shuttle init --from joshua-mo-143/shuttle-aws-templates --subfolder s3 |

Templates currently utilise a custom Shuttle plugin that lets you initialise an AWS `SdkConfig` that you can plug into any of the AWS SDK Rust crates.
