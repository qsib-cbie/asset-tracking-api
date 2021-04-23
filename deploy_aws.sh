aws cloudformation deploy --template-file "template.yaml" --stack-name "qsib-asset-tracking-api" \
    --parameter-overrides \
        ProjectName="qsib-asset-tracking-api" \
        VpcId="vpc-008ccf7159e4dd224" \
        VpcCIDR="10.0.0.0/16" \
        DBSubnet1="subnet-04f483713655db1cd" \
        DBSubnet2="subnet-0a26aadb34614ab3b" \
        DBSubnet3="subnet-098551910b310ea2f" \
        RDSAdminUser="AdminUser"
