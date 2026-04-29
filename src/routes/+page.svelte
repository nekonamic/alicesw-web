<script>
  import {
    Page,
    Navbar,
    Block,
    Searchbar,
    List,
    ListItem,
    BlockTitle,
    Segmented,
    SegmentedButton,
  } from 'konsta/svelte';
  import AuroraTitle from './AuroraTitle.svelte';

  let activeSegmented = $state(1);
</script>

<Page>
  <Navbar title="魔理沙书屋" />
  <Block>
    <AuroraTitle 
      title="Svelte 5 Rocks" 
      subtitle="Built with Runes and Love." 
    />
  </Block>
  <Block strong inset class="space-y-4">
    <Segmented strong rounded>
      <SegmentedButton
        active={activeSegmented === 1}
        onClick={() => (activeSegmented = 1)}
      >
        内容
      </SegmentedButton>
      <SegmentedButton
        active={activeSegmented === 2}
        onClick={() => (activeSegmented = 2)}
      >
        作者
      </SegmentedButton>
      <SegmentedButton
        active={activeSegmented === 3}
        onClick={() => (activeSegmented = 3)}
      >
        书名
      </SegmentedButton>
    </Segmented>
  </Block>
  <Block>
     <Searchbar/>
  </Block>
</Page>