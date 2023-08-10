terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  access_key = var.aws_access_key
  secret_key = var.aws_secret_key
  region     = "australia-southeast-2"
}

resource "aws_instance" "example" {
  ami           = "ami-0c55b159cbfafe1f0" # Amazon Linux 2 AMI
  instance_type = "t2.micro"              # free tier
}

# ECR Repository

resource "aws_ecr_repository" "example" {
  name                 = "example" # name of the repository
  image_tag_mutability = "MUTABLE" # can be changed

  image_scanning_configuration {
    scan_on_push = true
  }
}

output "ecr_repository_url" {
  value = aws_ecr_repository.example.repository_url
}

output "ec2_instance_public_ip" {
  value = aws_instance.example.public_ip
}
