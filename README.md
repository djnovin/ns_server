# 1. Intro

This is just a micro-service that generates messages that are proxied to sms/email broker service. It uses the powerful [Actix Web](https://actix.rs/) framework.

# 2. How to run?

Easiest way to spin up a working sample of this repo is via Docker Compose

## 2.1 Docker Compose

To spin up the entire stack on local, just run the following command (depending on your target architecture).

1. arm64 / aarch64, e.g. M1 Mac,

```bash
# It is supported by default. You can simply run
docker-compose up

# You may also specify it explicitly
docker-compose build --build-arg "ARCH=aarch64"
docker-compose up
```

2. amd84 / x86_64, e.g. Intel chips,

```bash
# You need to specify the ARCH argument accordingly
docker-compose build --build-arg "ARCH=x86_64"
docker-compose up
```

You need to have [Docker](https://www.docker.com/products/docker-desktop/) installed on your machine.

The components included in the compose file are

- API (Actix Web)

## 2.2 `.env` file

You can just make a copy from `.env.example` (the sample env file) and rename it as `.env`.

## 2.3 For Local Development

To develop the Actix Web application itself, you need to first stop the `ns-api` service if you did spun it up using docker-compose in the previous steps.

After that, you can start the application via the following command

```bash
cargo run
```

It works just like any other Rust applications that use [Cargo](https://doc.rust-lang.org/cargo/).

# 3. Infrastructure Setup

We use Terraform to manage the infrastructure that hosts our Rust microservice on an AWS EC2 instance and stores Docker images in an AWS Elastic Container Registry (ECR).

## Prerequisites

- **Terraform**: Please make sure you have Terraform installed. You can download it from the [official website](https://www.terraform.io/downloads.html).
- **AWS CLI**: Ensure that the AWS CLI is installed and configured with the necessary IAM permissions. Instructions for installation and configuration can be found on the [AWS CLI website](https://aws.amazon.com/cli/).
- **Docker**: You'll need Docker to build and push the container image.

## Setting Up Infrastructure

1. **Initialize Terraform**:

   Navigate to the directory containing the Terraform files and run the following command:

   ```bash
   terraform init
   ```

2. **Terraform Apply**

Next, apply the Terraform configuration to create the necessary AWS resources:

```bash
terraform apply
```

This command will output the ECR repository URL and the public IP address of the EC2 instance.

3. Build Docker Image:

Use the provided Dockerfile to build your Rust microservice:

```bash
docker build -t YOUR_ECR_REPOSITORY_URL/YOUR_IMAGE:TAG .
```

Replace YOUR_ECR_REPOSITORY_URL/YOUR_IMAGE:TAG with the ECR repository URL obtained from the Terraform output.

4. Push Docker Image:

Authenticate Docker with the ECR repository and push the image:

```bash
aws ecr get-login-password --region australia-southeast-2 | docker login --username AWS --password-stdin YOUR_ECR_REPOSITORY_URL
docker push YOUR_ECR_REPOSITORY_URL/YOUR_IMAGE:TAG
```

5. Verify the Deployment:

You can verify the deployment by accessing the microservice through the public IP of the EC2 instance, also provided in the Terraform output.

6. Destroy the Infrastructure:

Once you're done, you can destroy the infrastructure by running the following command:

```bash
terraform destroy
```

# 4. Things to do

| Items                                                     | Status               |
| --------------------------------------------------------- | -------------------- |
| Actix Routes                                              | :white_check_mark:   |
| Unit Tests                                                | :white_large_square: |
| Error Handling                                            | :white_large_square: |
| Containerization                                          | :white_check_mark:   |
| OpenAPI - with [utoipa](https://github.com/juhaku/utoipa) | :white_large_square: |
| Auth                                                      | :white_large_square: |
| CI/CD                                                     | :white_large_square: |
| ?                                                         | :white_large_square: |
