using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

public class EnemyUI : MonoBehaviour
{
    [Header( "Data" )]
    public TextMeshProUGUI NameText;

    public TextMeshProUGUI HPText;

    [Header( "Listening to Events" )]
    public IntEventChannelSO m_healthChange;

    public Entity observedEntity;

    // Start is called before the first frame update
    void Start()
    {
        NameText.text                =  observedEntity.entityData.id;
        m_healthChange.OnEventRaised += OnHealthChange;
        
        HPText.text = $"{observedEntity.CurrentHealth}/{observedEntity.Health}";
    }

    public void OnHealthChange( int[] numbers )
    {
        HPText.text = $"{numbers[ 0 ]}/{numbers[ 1 ]}";
    }

    // Update is called once per frame
    void Update()
    {
    }
}