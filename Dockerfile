FROM public.ecr.aws/lambda/provided:al2

ADD ./target/x86_64-unknown-linux-musl/release/atcoder-ogp ${LAMBDA_RUNTIME_DIR}/bootstrap
ADD ./templates/ ${LAMBDA_TASK_ROOT}/templates/

CMD [ "lambda-handler" ]