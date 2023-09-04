#[cfg(test)]
mod tests{

    use crate::entity::content::Content;
    

    #[test]
    fn it_must_build_a_instance_of_the_backup(){

        //arrange
        let data: String = String::from("dummy");
        let version: String = String::from("sha:x723-456");

        //action
        let backup = Content::new(data, version);

        //assert
        assert_eq!(backup.value, "dummy");
        assert_eq!(backup.version, "sha:x723-456");
    }

    #[test]
    fn it_must_be_a_valid_state(){
        
        //arrange
        let data: String = String::from("dummy");
        let version: String = String::from("sha:x723-456");

        //action
        let backup = Content::new(data, version);

        //assert
        assert_eq!(backup.is_valid(), true);
    }

    #[test]
    fn it_must_be_a_invalid_state(){
        
        //arrange
        let data: String = Default::default();
        let version: String = Default::default();

        //action
        let backup = Content::new(data, version);

        //assert
        assert_eq!(backup.is_valid(), false);
    }

}